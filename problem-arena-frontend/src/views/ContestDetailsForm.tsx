import {useNavigate, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {ContestService} from "../services/contest-service";
import {Contest, NewContest} from "../model/contest";

export default function ContestDetailsForm() {
    const navigate = useNavigate();
    const { id } = useParams();
    const [name, setName] = useState<string>("");
    const [description, setDescription] = useState<string>("");

    useEffect(() => {
        if (id !== undefined) {
            ContestService.getContest(id).then((res) => {
                let contest = res.data;
                setName(contest.name);
                setDescription(contest.description);
            });
        }
    }, [id]);

    function submit() {
        if (id != null) {
            const contest: Contest = {
                id: Number(id),
                name,
                description
            };

            ContestService.updateContest(contest).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("Contest was updated successfully!");
                    navigate(-1);
                }
            })
        }
        else {
            const contest: NewContest = {
                name,
                description
            };

            ContestService.addContest(contest).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("Contest was added successfully!");
                    navigate(-1);
                }
            })
        }
    }

    return (
        <div>
            <h1 className="title">{id != null? "Edit Contest": "Create Contest"}</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    <div className="field">
                        <label className="label">Name</label>
                        <div className="control">
                            <input className="input"
                                   type="text" placeholder="Contest name"
                                   value={name}
                                   onChange={(e) => setName(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Description</label>
                        <div className="control">
                            <textarea
                                className="textarea"
                                placeholder="Description"
                                value={description}
                                onChange={(e) => setDescription(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => submit()}>
                                {id != null? "Modify Contest": "Add Contest"}
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