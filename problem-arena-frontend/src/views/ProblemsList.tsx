import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {ProblemWithCreatorDTO} from "../model/problem";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<ProblemWithCreatorDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [filter, setFilter] = useState<number>();
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const [page, setPage] = useState<number>(1);
    const [numPages, setNumPages] = useState<number>(10);
    const navigate = useNavigate();

    useEffect(() => {
        ProblemsService.getProblems(pagination, filter).then((res) => {
            if (res.data.length > 0) {
                setProblemList(res.data);
            }
        });
        ProblemsService.getNumberOfProblems().then((res) => {
            setNumPages(Math.ceil(res.data / 10.0));
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
        if (problemList.length > 0  && page > 1) {
            setPagination({first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: -1});
            setPage(page - 1);
        }
    }

    const toPage = async (pg: number) => {
        if (pg <= 5) {
            let pagination = {first_id: -1, last_id: 0, limit: 10, direction: 1};
            let problems = (await ProblemsService.getProblems(pagination, filter)).data;

            for (let i = 2; i <= pg; i++) {
                pagination = {first_id: problems[0].id, last_id: problems[problems.length - 1].id, limit: 10, direction: 1};
                problems = (await ProblemsService.getProblems(pagination, filter)).data;
            }

            setPage(pg);
            setProblemList(problems);
        }
        else if (pg < page) {
            let pagination = {first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: -1};
            let problems = problemList;
            for(let i = page - 1; i >= pg; i--) {
                pagination = {first_id: problems[0].id, last_id: problems[problems.length - 1].id, limit: 10, direction: -1};
                problems = (await ProblemsService.getProblems(pagination, filter)).data;
            }

            setPage(pg);
            setProblemList(problems);
        }
        else if (numPages - pg <= 5) {
            let pagination = {first_id: 1000000000, last_id: 1000000000, limit: 10, direction: -1};
            let problems = (await ProblemsService.getProblems(pagination, filter)).data;
            for(let i = numPages - 1; i >= pg; i--) {
                pagination = {first_id: problems[0].id, last_id: problems[problems.length - 1].id, limit: 10, direction: -1};
                problems = (await ProblemsService.getProblems(pagination, filter)).data;
            }

            setPage(pg);
            setProblemList(problems);
        }
        else {
            let pagination = {first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: 1};
            let problems = problemList;
            for(let i = page + 1; i <= pg; i++) {
                pagination = {first_id: problems[0].id, last_id: problems[problems.length - 1].id, limit: 10, direction: 1};
                problems = (await ProblemsService.getProblems(pagination, filter)).data;
            }

            setPage(pg);
            setProblemList(problems);
        }
    }

    const nextPage = () => {
        if (problemList.length > 0 && page < numPages) {
            setPagination({first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: 1});
            setPage(page + 1);
        }
    }

    /*function sortByRating() {
        let x = JSON.parse(JSON.stringify(problemList));
        x = x.sort((a: Problem, b: Problem) => a.rating - b.rating);
        setProblemList(x);
    }*/

    const paginationComponentLeft = () => {
        let res = []
        for (let i = 1; i < Math.min(page, 6); i++) {
            res.push(<li><button className="pagination-link" aria-current="page" onClick={() => toPage(i)}>{i}</button></li>);
        }

        if (page - 5 > 6) {
            res.push(<li><span className="pagination-ellipsis">&hellip;</span></li>);
        }

        for (let i = Math.max(6, page - 5); i < page; i++) {
            res.push(<li><button className="pagination-link" aria-current="page" onClick={() => toPage(i)}>{i}</button></li>);
        }

        return res;
    };

    const paginationComponentRight = () => {
        let res = []
        for (let i = page + 1; i < Math.min(page + 6, numPages + 1); i++) {
            res.push(<li><button className="pagination-link" aria-current="page" onClick={() => toPage(i)}>{i}</button></li>);
        }

        if (page + 6 < numPages - 5) {
            res.push(<li><span className="pagination-ellipsis">&hellip;</span></li>);
        }

        for (let i = Math.max(numPages - 5, page + 6); i < numPages + 1; i++) {
            res.push(<li><button className="pagination-link" aria-current="page" onClick={() => toPage(i)}>{i}</button></li>);
        }

        return res;
    }

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
            <Table columns={["Name", "Author", "Contest", "Rating", "Submissions"]}
                   properties={["name", "author", "contest", "rating", "cnt"]}
                   elements={problemList}
                   path={"/problem"}
                   deleteFunction={(id) => deleteProblem(id)}
                   creator="creator"
                   uid="uid"
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
                <ul className="pagination-list">
                    {paginationComponentLeft()}
                    <li>
                        <button className="pagination-link is-current" aria-current="page">{page}</button>
                    </li>
                    {paginationComponentRight()}
                </ul>
            </nav>
        </div>
    );
}