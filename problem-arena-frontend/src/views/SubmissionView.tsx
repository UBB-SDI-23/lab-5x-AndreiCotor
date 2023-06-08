import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {SubmissionDTO} from "../model/submission";
import {SubmissionService} from "../services/submission-service";

export default function SubmissionView() {
    const { id } = useParams();
    const [submission, setSubmission] = useState<SubmissionDTO>();
    const [error, setError] = useState<string>("");

    useEffect(() => {
        if (id !== undefined) {
            SubmissionService.getSubmission(id).then((res) => setSubmission(res.data))
                .catch((res) => setError("An error has occurred!"))
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">Submission {submission?.id}</h1>
            <h2 className="subtitle">General Information</h2>
            <p className="has-text-danger">{error}</p>
            <table className="table is-fullwidth">
                <thead>
                <tr>
                    <th>Problem</th>
                    <th>User</th>
                    <th>Score</th>
                    <th>Programming language</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>{submission?.problem.name}</td>
                    <td>{submission?.user.last_name}</td>
                    <td>{submission?.score}</td>
                    <td>{submission?.language}</td>
                </tr>
                </tbody>
            </table>
            <h2 className="subtitle">Source code</h2>
            <p>{submission?.source_code}</p>
        </div>
    );
}