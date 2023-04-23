import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";
import {Contest} from "../model/contest";
import {ContestService} from "../services/contest-service";

export default function ContestList() {
    const [contestList, setContestList] = useState<Contest[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();

    useEffect(() => {
        ContestService.getContests(pagination).then((res) => {
            if (res.data.length > 0) {
                setContestList(res.data);
            }
        })
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteContest = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await ContestService.deleteContest(id);
            forceUpdate();
        }
    }

    const previousPage = () => {
        if (contestList.length > 0) {
            setPagination({first_id: contestList[0].id, last_id: contestList[contestList.length - 1].id, limit: 10, direction: -1});
        }
    }

    const nextPage = () => {
        if (contestList.length > 0) {
            setPagination({first_id: contestList[0].id, last_id: contestList[contestList.length - 1].id, limit: 10, direction: 1});
        }
    }

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Contest List</h1>
                </div>
                <div className="column">
                    <button className="button is-pulled-right is-link" onClick={() => navigate("/contest/create")}>
                        Add Contest
                    </button>
                </div>
            </div>
            <Table columns={["Name"]}
                   properties={["name"]}
                   elements={contestList}
                   path={"/contest"}
                   deleteFunction={(id) => deleteContest(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}