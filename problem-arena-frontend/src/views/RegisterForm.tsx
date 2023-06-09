import {useState} from "react";
import {useNavigate} from "react-router-dom";
import {AuthService} from "../services/auth-service";

export default function RegisterForm() {
    const [username, serUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [confPassword, setConfPassword] = useState<string>("");
    const [errors, setErrors] = useState<any>({});
    const [uuid, setUuid] = useState<String>();
    const navigate = useNavigate();

    const validatePassword = () => {
        if (password.length <= 8) {
            setErrors({password: "Password must be longer than 8 characters!"});
            return false;
        }

        setErrors({});
        return true;
    };

    const validateConfirmPassword = () => {
        if (password !== confPassword) {
            setErrors({confPassword: "Passwords must be identical!"});
            return false;
        }

        setErrors({});
        return true;
    }
    
    const register = async () => {
        if (!validateConfirmPassword() || !validatePassword()) {
            return;
        }

        try {
            let uuid = (await AuthService.register(username, password)).data;
            setUuid(uuid);
        }
        catch (err: any) {
            if (err.response) {
                setErrors({general: err.response.data});
            }
            else {
                setErrors({general: "An error has occurred!"});
            }
        }
    }

    return (
        <div>
            <h1 className="title">Register</h1>
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
                                   onBlur={() => validatePassword()}
                            />
                        </div>
                        {errors["password"]? (<p className="has-text-danger">{errors["password"]}</p>) : null}
                    </div>

                    <div className="field">
                        <label className="label">Confirm Password</label>
                        <div className="control">
                            <input className="input"
                                   type="password"
                                   placeholder="Confirm Password"
                                   value={confPassword}
                                   onChange={(e) => setConfPassword(e.target.value)}
                                   onBlur={() => validateConfirmPassword()}
                            />
                        </div>
                        {errors["confPassword"]? (<p className="has-text-danger">{errors["confPassword"]}</p>) : null}
                    </div>

                    <p>Already have an account? <a href="/login">Login instead.</a></p>
                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => register()}>
                                Register
                            </button>
                        </div>
                        <div className="control">
                            <button className="button is-link is-light" onClick={() => navigate(-1)}>
                                Cancel
                            </button>
                        </div>
                    </div>
                    {uuid? (<a href={"/register/confirm/" + uuid}>Click here to confirm your account</a>): null}
                </div>
            </div>
        </div>
    );
}