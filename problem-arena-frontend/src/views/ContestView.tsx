import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {Contest} from "../model/contest";
import {ContestService} from "../services/contest-service";

export default function ContestView() {
    const { id } = useParams();
    const [contest, setContest] = useState<Contest>();

    useEffect(() => {
        if (id !== undefined) {
            ContestService.getContest(id).then((res) => setContest(res.data))
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">{contest?.name}</h1>
            <h2 className="subtitle">General Information</h2>
            <table className="table is-fullwidth">
                <thead>
                <tr>
                    <th>Name</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>{contest?.name}</td>
                </tr>
                </tbody>
            </table>
            <h2 className="subtitle">Description</h2>
            <p>{contest?.description}</p>
        </div>
    );
}