import {useContext, useState} from "react";
import {useNavigate} from "react-router-dom";
import {AuthService} from "../services/auth-service";
import {saveLoginDTO} from "../model/LoginDTO";
import {AuthContext} from "../contexts/AuthContext";

export default function LoginForm() {
    const [username, serUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [errors, setErrors] = useState<any>({});
    const { setAuthContext } = useContext(AuthContext);
    const navigate = useNavigate();

    const login = async () => {
        try {
            let result = await AuthService.login(username, password);
            saveLoginDTO(result.data);
            setAuthContext(result.data);
            navigate("/");
        }
        catch (err: any) {
            if (err.response.status === 400) {
                if (err.response.data === "Invalid username!") {
                    setErrors({username: err.response.data});
                }
                else {
                    setErrors({password: err.response.data});
                }
            }
            else {
                setErrors({general: err.response.statusText});
            }
        }
    }

    return (
        <div>
            <h1 className="title">Login</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    {errors["general"]? (<p className="has-text-danger">{errors["general"]}</p>) : null}
                    <div className="field">
                        <label className="label">Username</label>
                        <div className="control">
                            <input className="input"
                                   type="text" placeholder="Username"
                                   value={username}
                                   onChange={(e) => serUsername(e.target.value)}
                            />
                        </div>
                        {errors["username"]? (<p className="has-text-danger">{errors["username"]}</p>) : null}
                    </div>

                    <div className="field">
                        <label className="label">Password</label>
                        <div className="control">
                            <input className="input"
                                   type="password"
                                   placeholder="Password"
                                   value={password}
                                   onChange={(e) => setPassword(e.target.value)}
                            />
                        </div>
                        {errors["password"]? (<p className="has-text-danger">{errors["password"]}</p>) : null}
                    </div>

                    <p>Don't have an account? <a href="/register">Create one instead.</a></p>
                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => login()}>
                                Login
                            </button>
                        </div>
                        <div className="control">
                            <button className="button is-link is-light" onClick={() => navigate(-1)}>
                                Cancel
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}