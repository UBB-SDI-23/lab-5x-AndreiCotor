import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {User} from "../model/user";
import {UserService} from "../services/user-service";

export default function UserView() {
    const { id } = useParams();
    const [user, setUser] = useState<User>();

    useEffect(() => {
        if (id !== undefined) {
            UserService.getUser(id).then((res) => setUser(res.data))
        }
    }, [id]);

    return (
        <div>
            <h1 className="title">{user?.last_name}</h1>
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
                    <td>{user?.last_name}</td>
                    <td>{user?.first_name}</td>
                    <td>{user?.school}</td>
                    <td>{user?.teacher}</td>
                </tr>
                </tbody>
            </table>
            <h2 className="subtitle">Bio</h2>
            <p>{user?.bio}</p>
        </div>
    );
}