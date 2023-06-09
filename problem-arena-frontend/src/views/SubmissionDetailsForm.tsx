import {useNavigate, useParams} from "react-router-dom";
import {useContext, useEffect, useState} from "react";
import {SubmissionService} from "../services/submission-service";
import {NewSubmission, Submission} from "../model/submission";
import {faSearch} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {UserService} from "../services/user-service";
import {User} from "../model/user";
import {ProblemsService} from "../services/problems-service";
import {Problem} from "../model/problem";
import {AuthContext} from "../contexts/AuthContext";

const WAIT_INTERVAL = 1000;
let timerIDU: string | number | NodeJS.Timeout | undefined;
let timerIDP: string | number | NodeJS.Timeout | undefined;

export default function SubmissionDetailsForm() {
    const navigate = useNavigate();
    const { id } = useParams();
    const [userId, setUserId] = useState<number>(-1);
    const [problemId, setProblemId] = useState<number>(-1);
    const [sourceCode, setSourceCode] = useState<string>("");
    const [score, setScore] = useState<number>(0);
    const [language, setLanguage] = useState<string>("");
    const [selectedUserName, setSelectedUserName] = useState<string>("");
    const [selectedProblem, setSelectedProblem] = useState<string>("");
    const [userSearch, setUserSearch] = useState<string>("");
    const [problemSearch, setProblemSearch] = useState<string>("");
    const [userList, setUserList] = useState<User[]>([]);
    const [problemList, setProblemList] = useState<Problem[]>([]);
    const [errors, setErrors] = useState<any>({});
    const { authContext } = useContext(AuthContext);

    useEffect(() => {
        if (id !== undefined) {
            SubmissionService.getSubmission(id).then((res) => {
                let submission = res.data;
                setUserId(submission.user.id);
                setProblemId(submission.problem.id);
                setSourceCode(submission.source_code);
                setScore(submission.score);
                setLanguage(submission.language);
                setSelectedUserName(submission.user.last_name);
                setSelectedProblem(submission.problem.name);
            });
        }
        handleAutocompleteUser("");
        handleAutocompleteProblem("");
    }, [id]);

    function submit() {
        if (score < 0 || score > 100) {
            setErrors({score: true});
            return;
        }

        let val;
        if (userId === -1 && authContext) {
            val = authContext.id;
        }
        else {
            val = userId;
        }

        if (id != null) {
            const submission: Submission = {
                id: Number(id),
                user_id: val,
                problem_id: problemId,
                source_code: sourceCode,
                score,
                language
            };

            SubmissionService.updateSubmission(submission).then((res) => {
                navigate(-1);
            }).then((res) => setErrors({general: "An error has occurred!"}))
        }
        else {
            const submission: NewSubmission = {
                user_id: val,
                problem_id: problemId,
                source_code: sourceCode,
                score,
                language
            };

            SubmissionService.addSubmission(submission).then((res) => {
                navigate(-1);
            }).then((res) => setErrors({general: "An error has occurred!"}))
        }
    }

    const handleAutocompleteUser = (user: string) => {
        clearTimeout(timerIDU);

        timerIDU = setTimeout(() => {
            UserService.getUsersAutocomplete(user).then((res) => {
                setUserList(res.data);
            });
        }, WAIT_INTERVAL);
    }

    const handleAutocompleteProblem = (name: string) => {
        clearTimeout(timerIDP);

        timerIDP = setTimeout(() => {
            ProblemsService.getProblemsAutocomplete(name).then((res) => {
                setProblemList(res.data);
            });
        }, WAIT_INTERVAL);
    }

    const userNameList = userList.map((el) => {
       return (
           <p className="panel-block" key={el.id} onClick={() => {setUserId(el.id); setSelectedUserName(el.last_name)}}>
               {el.last_name}
           </p>
       );
    });

    const problemNameList = problemList.map((el) => {
        return (
            <p className="panel-block" key={el.id} onClick={() => {setProblemId(el.id); setSelectedProblem(el.name)}}>
                {el.name}
            </p>
        );
    });

    return (
        <div>
            <h1 className="title">{id != null? "Edit Submission": "Create Submission"}</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    {errors["general"]? (<p className="has-text-danger">{errors["general"]}</p>) : null}
                    {(authContext && authContext.role === "regular")? null : (
                        <nav className="panel">
                            <p className="panel-heading">
                                User
                            </p>
                            <div className="panel-block">
                                <p className="control has-icons-left">
                                    <input className="input"
                                           type="text"
                                           placeholder="Search"
                                           value={userSearch}
                                           onChange={(e) => {
                                               setUserSearch(e.target.value);
                                               handleAutocompleteUser(e.target.value)
                                           }}
                                    />
                                    <span className="icon is-left">
                                  <FontAwesomeIcon icon={faSearch}/>
                              </span>
                                </p>
                            </div>
                            {(userNameList.length > 0) ? userNameList : (<p>No data to show.</p>)}

                            <div className="panel-block">
                                <p>Selected user: {selectedUserName} </p>
                            </div>
                        </nav>
                    )}

                    <nav className="panel">
                        <p className="panel-heading">
                            Problem
                        </p>

                        <div className="panel-block">
                            <p className="control has-icons-left">
                                <input className="input"
                                       type="text"
                                       placeholder="Search"
                                       value={problemSearch}
                                       onChange={(e) => {setProblemSearch(e.target.value); handleAutocompleteProblem(e.target.value)}}
                                />
                                <span className="icon is-left">
                                  <FontAwesomeIcon icon={faSearch} />
                              </span>
                            </p>
                        </div>
                        {(problemNameList.length > 0) ? problemNameList : (<p>No data to show.</p>)}
                        <div className="panel-block">
                            <p>Selected problem: {selectedProblem} </p>
                        </div>
                    </nav>

                    <div className="field">
                        <label className="label">Language</label>
                        <div className="control">
                            <input className="input"
                                   type="text"
                                   placeholder="Language"
                                   value={language}
                                   onChange={(e) => setLanguage(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field">
                        <label className="label">Score</label>
                        <div className="control">
                            <input className="input"
                                   type="number"
                                   placeholder="Score"
                                   value={score}
                                   onChange={(e) => setScore(Number(e.target.value))}
                            />
                        </div>
                        {errors["score"]? (<p className="has-text-danger">Score must be between 0 and 100!</p>) : null}
                    </div>

                    <div className="field">
                        <label className="label">Source Code</label>
                        <div className="control">
                            <textarea
                                className="textarea"
                                placeholder="Source Code"
                                value={sourceCode}
                                onChange={(e) => setSourceCode(e.target.value)}
                            />
                        </div>
                    </div>

                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => submit()}>
                                {id != null? "Modify Submission": "Add Submission"}
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