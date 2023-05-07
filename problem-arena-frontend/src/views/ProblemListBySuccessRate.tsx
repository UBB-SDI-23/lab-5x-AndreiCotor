import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {ProblemStatisticsDTO} from "../model/problem";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {StatisticPagination} from "../model/PaginationDTO";

export default function ProblemListBySuccessRate() {
    const [problemList, setProblemList] = useState<ProblemStatisticsDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<StatisticPagination>({first_id: -1, first_stat: -1, last_id: 0, last_stat: 0, limit: 10, direction: 1});
    const navigate = useNavigate();

    useEffect(() => {
        ProblemsService.getProblemsBySuccessRate(pagination).then((res) => {
            if (res.data.length > 0) {
                setProblemList(res.data);
            }
        })
    }, [value, pagination]);

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
            setPagination({
                first_id: problemList[0].id,
                first_stat: problemList[0].cnt,
                last_id: problemList[problemList.length - 1].id,
                last_stat: problemList[problemList.length - 1].cnt,
                limit: 10,
                direction: -1
            });
        }
    }

    const nextPage = () => {
        if (problemList.length > 0) {
            setPagination({
                first_id: problemList[0].id,
                first_stat: problemList[0].cnt,
                last_id: problemList[problemList.length - 1].id,
                last_stat: problemList[problemList.length - 1].cnt,
                limit: 10,
                direction: 1
            });
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
            </nav>
        </div>
    );
}