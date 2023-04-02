import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {Problem} from "../model/problem";
import RatingDisplay from "../components/RatingDisplay";

export default function ProblemView() {
    const { id } = useParams();
    const [problem, setProblem] = useState<Problem>();

    useEffect(() => {
        if (id !== undefined) {
            ProblemsService.getProblem(id).then((res) => setProblem(res.data))
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">{problem?.name}</h1>
            <h2 className="subtitle">General Information</h2>
            <table className="table is-fullwidth">
                <thead>
                    <tr>
                        <th>Author</th>
                        <th>Contest</th>
                        <th>Rating</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{problem?.author}</td>
                        <td>{problem?.contest}</td>
                        <td><RatingDisplay rating={problem?.rating}/></td>
                    </tr>
                </tbody>
            </table>
            <h2 className="subtitle">Statement</h2>
            <p>{problem?.statement}</p>
        </div>
    );
}