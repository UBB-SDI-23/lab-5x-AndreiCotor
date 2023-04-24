import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {Problem} from "../model/problem";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<Problem[]>([]);
    const [value, setValue] = useState<number>(0);
    const [filter, setFilter] = useState<number>();
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();

    useEffect(() => {
        ProblemsService.getProblems(pagination, filter).then((res) => {
            if (res.data.length > 0) {
                setProblemList(res.data);
            }
        })
    }, [value, pagination, filter]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteProblem = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await ProblemsService.deleteProblem(id);
            forceUpdate();
        }
    }

    const previousPage = () => {
        if (problemList.length > 0) {
            setPagination({first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: -1});
        }
    }

    const nextPage = () => {
        if (problemList.length > 0) {
            setPagination({first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: 1});
        }
    }

    /*function sortByRating() {
        let x = JSON.parse(JSON.stringify(problemList));
        x = x.sort((a: Problem, b: Problem) => a.rating - b.rating);
        setProblemList(x);
    }*/

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Problem List</h1>
                </div>
                <div className="column">
                    <button className="button is-pulled-right is-link" onClick={() => navigate("/problem/create")}>
                        Add Problem
                    </button>
                </div>
            </div>
            <div className="field">
                <label className="label">Rating larger than</label>
                <div className="control">
                    <input className="input"
                           type="number"
                           placeholder="Rating larger than"
                           value={filter}
                           onChange={(e) => setFilter(Number(e.target.value))}
                    />
                </div>
            </div>
            <Table columns={["Name", "Author", "Contest", "Rating"]}
                   properties={["name", "author", "contest", "rating"]}
                   elements={problemList}
                   path={"/problem"}
                   deleteFunction={(id) => deleteProblem(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}