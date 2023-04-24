import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {Participation} from "../model/participates";
import {ParticipationService} from "../services/participates-service";
import {User} from "../model/user";
import {Contest} from "../model/contest";
import {UserService} from "../services/user-service";
import {ContestService} from "../services/contest-service";

export default function ParticipationView() {
    const { uid, cid } = useParams();
    const [participation, setParticipation] = useState<Participation>();
    const [user, setUser] = useState<User>();
    const [contest, setContest] = useState<Contest>();

    useEffect(() => {
        if (uid !== undefined) {
            ParticipationService.getParticipation(uid + "/" + cid).then((res) => setParticipation(res.data));
            UserService.getUser(uid).then((res) => setUser(res.data));
            if (cid !== undefined) {
                ContestService.getContest(cid).then((res) => setContest(res.data));
            }
        }
    }, [uid, cid]);

    return (
        <div>
            <h1 className="title">Participation</h1>
            <h2 className="subtitle">General Information</h2>
            <table className="table is-fullwidth">
                <thead>
                <tr>
                    <th>User</th>
                    <th>Contest</th>
                    <th>Score</th>
                    <th>Official</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>{user?.last_name}</td>
                    <td>{contest?.name}</td>
                    <td>{String(participation?.score)}</td>
                    <td>{String(participation?.official)}</td>
                </tr>
                </tbody>
            </table>
        </div>
    );
}