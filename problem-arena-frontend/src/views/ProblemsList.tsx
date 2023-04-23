import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {Problem} from "../model/problem";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<Problem[]>([]);
    const [value, setValue] = useState<number>(0);
    const navigate = useNavigate();

    useEffect(() => {
        ProblemsService.getProblems().then((res) => setProblemList(res.data))
    }, [value]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteProblem = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await ProblemsService.deleteProblem(id);
            forceUpdate();
        }
    }

    /*function sortByRating() {
        let x = JSON.parse(JSON.stringify(problemList));
        x = x.sort((a: Problem, b: Problem) => a.rating - b.rating);
        setProblemList(x);
    }*/

    return (
        <div>
            <div className="columns">
                <div className="column">
                    <h1 className="title">Problem List</h1>
                </div>
                <div className="column">
                    <button className="button is-pulled-right mr-2 is-link" onClick={() => navigate("/problem/create")}>
                        Add Problem
                    </button>
                </div>
            </div>
            <Table columns={["Name", "Author", "Contest", "Rating"]}
                   properties={["name", "author", "contest", "rating"]}
                   elements={problemList}
                   path={"/problem"}
                   deleteFunction={(id) => deleteProblem(id)}
            />
        </div>
    );
}