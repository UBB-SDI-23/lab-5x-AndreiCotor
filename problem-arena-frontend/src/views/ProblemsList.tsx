import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {ProblemStatisticsDTO} from "../model/problem";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<ProblemStatisticsDTO[]>([]);
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

    const firstPage = () => {
        setPagination({first_id: -1, last_id: 0, limit: 10, direction: 1});
        setPage(1);
    }

    const nextPage = () => {
        if (problemList.length > 0 && page < numPages) {
            setPagination({first_id: problemList[0].id, last_id: problemList[problemList.length - 1].id, limit: 10, direction: 1});
            setPage(page + 1);
        }
    }

    const lastPage = () => {
        setPagination({first_id: 1000000000, last_id: 1000000000, limit: 10, direction: -1});
        setPage(numPages);
    }

    /*function sortByRating() {
        let x = JSON.parse(JSON.stringify(problemList));
        x = x.sort((a: Problem, b: Problem) => a.rating - b.rating);
        setProblemList(x);
    }*/

    const paginationComponentLeft = () => {
        if (page === 1) {
            return null;
        }
        else if (page === 2) {
            return (<li><button className="pagination-link" aria-current="page" onClick={() => previousPage()}>{page - 1}</button></li>)
        }
        else if (page === 3) {
            return (
                <>
                    <li><button className="pagination-link" aria-current="page" onClick={() => firstPage()}>{page - 2}</button></li>
                    <li><button className="pagination-link" aria-current="page" onClick={() => previousPage()}>{page - 1}</button></li>
                </>
            );
        }
        else {
            return (
                <>
                    <li><button className="pagination-link" aria-current="page" onClick={() => firstPage()}>1</button></li>
                    <li><span className="pagination-ellipsis">&hellip;</span></li>
                    <li><button className="pagination-link" aria-current="page" onClick={() => previousPage()}>{page - 1}</button></li>
                </>
            );
        }
    };

    const paginationComponentRight = () => {
        if (page === numPages) {
            return null;
        }
        else if (page === numPages - 1) {
            return (<li><button className="pagination-link" aria-current="page" onClick={() => nextPage()}>{page + 1}</button></li>)
        }
        else if (page === numPages - 2) {
            return (
                <>
                    <li><button className="pagination-link" aria-current="page" onClick={() => nextPage()}>{page + 1}</button></li>
                    <li><button className="pagination-link" aria-current="page" onClick={() => lastPage()}>{page + 2}</button></li>
                </>
            );
        }
        else {
            return (
                <>
                    <li><button className="pagination-link" aria-current="page" onClick={() => nextPage()}>{page + 1}</button></li>
                    <li><span className="pagination-ellipsis">&hellip;</span></li>
                    <li><button className="pagination-link" aria-current="page" onClick={() => lastPage()}>{numPages}</button></li>
                </>
            );
        }
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