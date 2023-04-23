import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {ParticipationPaginationDTO} from "../model/PaginationDTO";
import {Participation} from "../model/participates";
import {ParticipationService} from "../services/participates-service";

export default function ParticipationList() {
    const [participationList, setParticipationList] = useState<Participation[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<ParticipationPaginationDTO>({
        first_uid: -1,
        first_cid: -1,
        last_uid: 0,
        last_cid: 0,
        limit: 2,
        direction: 1
    });
    const navigate = useNavigate();

    useEffect(() => {
        ParticipationService.getParticipations(pagination).then((res) => {
            if (res.data.length > 0) {
                setParticipationList(res.data);
            }
        })
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteParticipation = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await ParticipationService.deleteParticipation(id);
            forceUpdate();
        }
    }

    const previousPage = () => {
        if (participationList.length > 0) {
            setPagination({
                first_uid: participationList[0].uid,
                first_cid: participationList[0].cid,
                last_uid: participationList[participationList.length - 1].uid,
                last_cid: participationList[participationList.length - 1].cid,
                limit: 2,
                direction: -1
            });
        }
    }

    const nextPage = () => {
        if (participationList.length > 0) {
            setPagination({first_uid: participationList[0].uid,
                first_cid: participationList[0].cid,
                last_uid: participationList[participationList.length - 1].uid,
                last_cid: participationList[participationList.length - 1].cid,
                limit: 2,
                direction: 1
            });
        }
    }

    const participationsWithURLid = participationList.map((el) => ({
        id: String(el.uid) + "/" + String(el.cid),
        official: String(el.official),
        score: el.score,
        uid: el.uid,
        cid: el.cid
    }));

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Participation List</h1>
                </div>
                <div className="column">
                    <button className="button is-pulled-right is-link" onClick={() => navigate("/participation/create")}>
                        Add Participation
                    </button>
                </div>
            </div>
            <Table columns={["Official", "Score"]}
                   properties={["official", "score"]}
                   elements={participationsWithURLid}
                   path={"/participation"}
                   deleteFunction={(id) => deleteParticipation(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}