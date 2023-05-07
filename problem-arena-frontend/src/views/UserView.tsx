import {useNavigate, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {UserPageDTO} from "../model/user";
import {UserService} from "../services/user-service";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPenToSquare} from "@fortawesome/free-solid-svg-icons";

export default function UserView() {
    const { id } = useParams();
    const navigate = useNavigate();
    const [user, setUser] = useState<UserPageDTO>();

    useEffect(() => {
        if (id !== undefined) {
            UserService.getUser(id).then((res) => setUser(res.data))
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">{user?.last_name}</h1>
            <h2 className="subtitle">General Information</h2>
            <button className="button is-link ml-2" onClick={() => navigate( "/user/edit/" + id)}>
                <FontAwesomeIcon icon={faPenToSquare} />
            </button>
            <table className="table is-fullwidth">
                <thead>
                <tr>
                    <th>Username</th>
                    <th>Last Name</th>
                    <th>First Name</th>
                    <th>School</th>
                    <th>Teacher</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>{user?.username}</td>
                    <td>{user?.last_name}</td>
                    <td>{user?.first_name}</td>
                    <td>{user?.school}</td>
                    <td>{user?.teacher}</td>
                </tr>
                </tbody>
            </table>
            <h2 className="subtitle">Bio</h2>
            <p>{user?.bio}</p>
            <h2 className="subtitle">Statistics</h2>
            <p>Problems proposed: {user?.problems_proposed}</p>
            <p>Contests added: {user?.contests_created}</p>
            <p>Submissions sent: {user?.submissions_sent}</p>
            <p>Participations: {user?.participations}</p>
        </div>
    );
}