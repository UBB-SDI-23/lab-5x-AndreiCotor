import {useNavigate, useParams} from "react-router-dom";
import {useContext, useEffect, useState} from "react";
import {UserPageDTO} from "../model/user";
import {UserService} from "../services/user-service";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPenToSquare} from "@fortawesome/free-solid-svg-icons";
import {AuthContext} from "../contexts/AuthContext";

export default function UserView() {
    const { id } = useParams();
    const navigate = useNavigate();
    const [user, setUser] = useState<UserPageDTO>();
    const { authContext } = useContext(AuthContext);
    const [error, setError] = useState<string>("");

    useEffect(() => {
        if (id !== undefined) {
            UserService.getUser(id).then((res) => setUser(res.data))
                .catch((res) => setError("An error has occurred!"));
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">{user?.last_name}</h1>
            <h2 className="subtitle">General Information</h2>
            {(!authContext || !id || (authContext.role === "regular" && authContext.id !== Number(id)))? null :
                (<button className="button is-link ml-2" onClick={() => navigate( "/user/edit/" + id)}>
                    <FontAwesomeIcon icon={faPenToSquare} />
                </button>)
            }
            <p className="has-text-danger">{error}</p>
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