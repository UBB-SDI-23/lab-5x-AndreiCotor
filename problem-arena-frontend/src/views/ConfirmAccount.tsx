import {useNavigate, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {AuthService} from "../services/auth-service";

export default function ConfirmAccount() {
    const navigate = useNavigate();
    const { uuid } = useParams();
    const [ status, setStatus ] = useState<string>("");

    useEffect(() => {
        if (uuid) {
            AuthService.confirm(uuid).then((res) => {
                setStatus("Account confirmed successfully!");
            }).catch((res) => {
                setStatus("We were unable to confirm your account!");
            });
        }
    }, );

    return (
        <div>
            {status? (<p>{status}</p>): null }
            <button className="button is-link is-light" onClick={() => navigate("/")}>
                Home
            </button>
        </div>
    );
}