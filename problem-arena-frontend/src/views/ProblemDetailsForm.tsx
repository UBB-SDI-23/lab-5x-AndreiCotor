import {useNavigate, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import {NewProblem, Problem} from "../model/problem";

export default function ProblemDetailsForm() {
    const navigate = useNavigate();
    const { id } = useParams();
    const [name, setName] = useState<string>("");
    const [author, setAuthor] = useState<string>("");
    const [contest, setContest] = useState<string>("");
    const [rating, setRating] = useState<number>(1);
    const [statement, setStatement] = useState<string>("");
    const [errors, setErrors] = useState<any>({});

    useEffect(() => {
        if (id !== undefined) {
            ProblemsService.getProblem(id).then((res) => {
               let problem = res.data;
               setName(problem.name);
               setAuthor(problem.author);
               setContest(problem.contest);
               setRating(problem.rating);
               setStatement(problem.statement);
            });
        }
    }, [id]);

    function submit() {
        if (rating < 0 || rating > 5) {
            setErrors({rating: true});
            return;
        }

        if (name.length < 3) {
            setErrors({name: true});
            return;
        }

        if (statement.length === 0) {
            setErrors({statement: true});
            return;
        }

        if (id != null) {
            const problem: Problem = {
                id: Number(id),
                name,
                author,
                contest,
                rating,
                statement
            };

            ProblemsService.updateProblem(problem).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("Problem was updated successfully!");
                    navigate(-1);
                }
            })
        }
        else {
            const problem: NewProblem = {
                name,
                author,
                contest,
                statement,
                rating
            };

            ProblemsService.addProblem(problem).then((res) => {
                if (res.status !== 200) {
                    alert(res.statusText);
                }
                else {
                    alert("Problem was added successfully!");
                    navigate(-1);
                }
            })
        }
    }

    return (
        <div>
            <h1 className="title">{id != null? "Edit Problem": "Create Problem"}</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    <div className="field">
                        <label className="label">Name</label>
                        <div className="control">
                            <input className="input"
                                   type="text" placeholder="Problem name"
                                   value={name}
                                   onChange={(e) => setName(e.target.value)}
                            />
                        </div>
                        {errors["name"]? (<p className="has-text-danger">Name must be longer than 3 characters!</p>) : null}
                    </div>

                    <div className="field">
                        <label className="label">Author</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="Author name"
                                   value={author}
                                   onChange={(e) => setAuthor(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Contest</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="Contest"
                                   value={contest}
                                   onChange={(e) => setContest(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Rating</label>
                        <div className="control">
                            <div className="select">
                                <select value={rating} onChange={(e) => setRating(Number(e.target.value))}>
                                    <option value={1}>1</option>
                                    <option value={2}>2</option>
                                    <option value={3}>3</option>
                                    <option value={4}>4</option>
                                    <option value={5}>5</option>
                                </select>
                            </div>
                        </div>
                        {errors["rating"]? (<p className="has-text-danger">Rating must be between 0 and 5!</p>) : null}
                    </div>

                    <div className="field">
                        <label className="label">Statement</label>
                        <div className="control">
                            <textarea
                                className="textarea"
                                placeholder="Statement"
                                value={statement}
                                onChange={(e) => setStatement(e.target.value)}
                            />
                        </div>
                        {errors["statement"]? (<p className="has-text-danger">Statement can't be empty!</p>) : null}
                    </div>

                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => submit()}>
                                {id != null? "Modify Problem": "Add Problem"}
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