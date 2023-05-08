import {useNavigate, useParams} from "react-router-dom";
import {useContext, useEffect, useState} from "react";
import {UserService} from "../services/user-service";
import {NewUser, User} from "../model/user";
import {AuthContext} from "../contexts/AuthContext";
import {AdminService} from "../services/admin-service";

export default function UserDetailsForm() {
    const navigate = useNavigate();
    const { id } = useParams();
    const [firstName, setFirstName] = useState<string>("");
    const [lastName, setLastName] = useState<string>("");
    const [school, setSchool] = useState<string>("");
    const [bio, setBio] = useState<string>("");
    const [teacher, setTeacher] = useState<string>("");
    const [role, setRole] = useState<string>("");
    const { authContext } = useContext(AuthContext);

    useEffect(() => {
        if (id !== undefined) {
            UserService.getUser(id).then((res) => {
                let user = res.data;
                setFirstName(user.first_name);
                setLastName(user.last_name);
                setSchool(user.school);
                setBio(user.bio);
                setTeacher(user.teacher);
                setRole(user.role);
            });
        }
    }, [id]);

    function submit() {
        if (id != null) {
            if (authContext && authContext.role === "admin") {
                AdminService.changeRole(Number(id), role).then(() => {navigate(-1)});
            }

            const user: User = {
                id: Number(id),
                first_name: firstName,
                last_name: lastName,
                school,
                bio,
                teacher
            };

            UserService.updateUser(user).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("User was updated successfully!");
                    navigate(-1);
                }
            })
        }
        else {
            const user: NewUser = {
                first_name: firstName,
                last_name: lastName,
                school,
                bio,
                teacher
            };

            UserService.addUser(user).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("User was added successfully!");
                    navigate(-1);
                }
            })
        }
    }

    return (
        <div>
            <h1 className="title">{id != null? "Edit User": "Create User"}</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    {(authContext && authContext.role === "admin")?
                        (<div className="field">
                            <label className="label">Role</label>
                            <div className="select">
                                <div className="control">
                                    <select value={role} onChange={(e) => setRole(e.target.value)}>
                                        <option value={"regular"}>Regular</option>
                                        <option value={"moderator"}>Moderator</option>
                                        <option value={"admin"}>Admin</option>
                                    </select>
                                </div>
                            </div>
                        </div>) : null
                    }

                    <div className="field">
                        <label className="label">First name</label>
                        <div className="control">
                            <input className="input"
                                   type="text" placeholder="First name"
                                   value={firstName}
                                   onChange={(e) => setFirstName(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Last Name</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="Last name"
                                   value={lastName}
                                   onChange={(e) => setLastName(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">School</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="School"
                                   value={school}
                                   onChange={(e) => setSchool(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Teacher</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="Teacher"
                                   value={teacher}
                                   onChange={(e) => setTeacher(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Bio</label>
                        <div className="control">
                            <textarea
                                className="textarea"
                                placeholder="Bio"
                                value={bio}
                                onChange={(e) => setBio(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => submit()}>
                                {id != null? "Modify User": "Add User"}
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