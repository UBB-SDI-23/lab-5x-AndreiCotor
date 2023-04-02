import {useCallback, useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {Problem} from "../model/problem";
import {useNavigate} from "react-router-dom";
import RatingDisplay from "../components/RatingDisplay";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faTrash, faPenToSquare} from "@fortawesome/free-solid-svg-icons";

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<Problem[]>([]);
    const [state, updateState] = useState<any>();
    const navigate = useNavigate();
    const forceUpdate = useCallback(() => updateState({}), []);

    useEffect(() => {
        ProblemsService.getProblems().then((res) => setProblemList(res.data))
    }, []);

    const deleteProblem = async (id: string) => {
        await ProblemsService.deleteProblem(id);
        forceUpdate();
    }

    const tableRows = problemList.map((el, index) => {
        return (<tr key={index}>
            <td onClick={() => navigate("/problem/" + el.id)}>{index + 1}</td>
            <td onClick={() => navigate("/problem/" + el.id)}>{el.name}</td>
            <td onClick={() => navigate("/problem/" + el.id)}>{el.author}</td>
            <td onClick={() => navigate("/problem/" + el.id)}>{el.contest}</td>
            <td onClick={() => navigate("/problem/" + el.id)}><RatingDisplay rating={el.rating} /></td>
            <td>
                <button className="button is-danger" onClick={() => deleteProblem(String(el.id))}>
                    <FontAwesomeIcon icon={faTrash} />
                </button>
                <button className="button is-link ml-2" onClick={() => navigate("/problem/edit/" + el.id)}>
                    <FontAwesomeIcon icon={faPenToSquare} />
                </button>
            </td>
        </tr>);
    });

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
            <table className="table is-hoverable is-fullwidth ">
                <thead>
                    <tr>
                        <th><abbr title="Index">#</abbr></th>
                        <th>Name</th>
                        <th>Author</th>
                        <th>Contest</th>
                        <th>Rating</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {tableRows}
                </tbody>
            </table>
        </div>
    );
}