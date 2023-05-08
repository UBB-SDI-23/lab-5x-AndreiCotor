import {useContext, useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {ParticipationPaginationDTO} from "../model/PaginationDTO";
import {ParticipationWithName} from "../model/participates";
import {ParticipationService} from "../services/participates-service";
import {AuthContext} from "../contexts/AuthContext";

export default function ParticipationList() {
    const [participationList, setParticipationList] = useState<ParticipationWithName[]>([]);
    const [value, setValue] = useState<number>(0);
    const [error, setError] = useState<string>("");
    const [pagination, setPagination] = useState<ParticipationPaginationDTO>({
        first_uid: -1,
        first_cid: -1,
        last_uid: 0,
        last_cid: 0,
        limit: 10,
        direction: 1
    });
    const navigate = useNavigate();
    const { authContext } = useContext(AuthContext);

    useEffect(() => {
        ParticipationService.getParticipations(pagination).then((res) => {
            if (res.data.length > 0) {
                setParticipationList(res.data.map((el) => {
                    return {
                        uid: el.uid,
                        cid: el.cid,
                        score: el.score,
                        official: el.official,
                        user: el.user.last_name,
                        contest: el.contest.name
                    }
                }));
            }
        }).catch((res) => {
            setError("An error has occurred!");
        })
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteParticipation = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            try {
                await ParticipationService.deleteParticipation(id);
            }
            catch (err) {
                setError("An error has occurred!");
            }
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
                limit: 10,
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
                limit: 10,
                direction: 1
            });
        }
    }

    const participationsWithURLid = participationList.map((el) => ({
        id: String(el.uid) + "/" + String(el.cid),
        official: String(el.official),
        score: el.score,
        uid: el.uid,
        cid: el.cid,
        user: el.user,
        contest: el.contest
    }));

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Participation List</h1>
                </div>
                <div className="column">
                    {authContext? (<button className="button is-pulled-right is-link" onClick={() => navigate("/participation/create")}>
                        Add Participation
                    </button>): null}
                </div>
            </div>
            <p className="has-text-danger">{error}</p>
            <Table columns={["Official", "Score", "Contest"]}
                   properties={["official", "score", "contest"]}
                   elements={participationsWithURLid}
                   path={"/participation"}
                   deleteFunction={(id) => deleteParticipation(id)}
                   creator="user"
                   uid="uid"
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}