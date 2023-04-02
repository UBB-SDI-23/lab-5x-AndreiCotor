import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import Problem from "../model/problem";

export default function ProblemView() {
    const { id } = useParams();
    const [problem, setProblem] = useState<Problem>();

    useEffect(() => {
        if (id !== undefined) {
            ProblemsService.getProblem(id).then((res) => setProblem(res.data))
        }
    }, [id]);

    return (
      <p>{problem?.name}</p>
    );
}