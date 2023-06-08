import {useContext, useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";
import {ContestWithCreatorDTO} from "../model/contest";
import {ContestService} from "../services/contest-service";
import {AuthContext} from "../contexts/AuthContext";

export default function ContestList() {
    const [contestList, setContestList] = useState<ContestWithCreatorDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();
    const [error, setError] = useState<string>("");
    const { authContext } = useContext(AuthContext);

    useEffect(() => {
        ContestService.getContests(pagination).then((res) => {
            if (res.data.length > 0) {
                setContestList(res.data);
            }
        }).catch((res) => setError("An error has occurred!"))
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteContest = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            try {
                await ContestService.deleteContest(id);
            }
            catch (err) {
                setError("An error has occurred!");
            }
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

    const firstPage = () => {
        setPagination({first_id: -1, last_id: 0, limit: 10, direction: 1});
    }

    const lastPage = () => {
        setPagination({first_id: 1000000000, last_id: 1000000000, limit: 10, direction: -1});
    }

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Contest List</h1>
                </div>
                <div className="column">
                    {authContext? (<button className="button is-pulled-right is-link" onClick={() => navigate("/contest/create")}>
                        Add Contest
                    </button>): null}
                </div>
            </div>
            <br/>
            <p className="has-text-danger">{error}</p>
            <Table columns={["Name", "Participants"]}
                   properties={["name", "cnt"]}
                   elements={contestList}
                   path={"/contest"}
                   deleteFunction={(id) => deleteContest(id)}
                   uid="uid"
                   creator="creator"
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
                <ul className="pagination-list">
                    <button className="pagination-link" onClick={() => firstPage()}>First page</button>
                    <button className="pagination-link" onClick={() => lastPage()}>Last page</button>
                </ul>
            </nav>
        </div>
    );
}