import {useContext} from "react";
import {AuthContext} from "../contexts/AuthContext";
import {clearLoginDTO} from "../model/LoginDTO";

export default function AccountComponent() {
    const { authContext, setAuthContext } = useContext(AuthContext);

    const logout = () => {
        clearLoginDTO();
        setAuthContext(null);
    };

    return (
        <div>
            {(!authContext)? (<div>
                    <a className="button is-info mr-2" href="/login">Login</a>
                    <a className="button is-info mr-2" href="/register">Register</a>
                </div>):
                <div>
                    <a className="mr-2" href={"/user/" + authContext.id}>Welcome, {authContext.username}!</a>
                    <button className="button is-info mr-2" onClick={() => logout()}>Logout</button>
                </div>}
        </div>
    );
}