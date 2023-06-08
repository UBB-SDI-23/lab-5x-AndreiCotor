import {useNavigate, useParams} from "react-router-dom";
import {useContext, useEffect, useState} from "react";
import {faSearch} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {UserService} from "../services/user-service";
import {User} from "../model/user";
import {Contest} from "../model/contest";
import {ParticipationService} from "../services/participates-service";
import {Participation} from "../model/participates";
import {ContestService} from "../services/contest-service";
import {AuthContext} from "../contexts/AuthContext";

const WAIT_INTERVAL = 1000;
let timerIDU: string | number | NodeJS.Timeout | undefined;
let timerIDC: string | number | NodeJS.Timeout | undefined;

export default function ParticipatesDetailsForm() {
    const { authContext } = useContext(AuthContext);
    const navigate = useNavigate();
    const { uid, cid } = useParams();
    const [userId, setUserId] = useState<number>(-1);
    const [contestId, setContestId] = useState<number>(-1);
    const [score, setScore] = useState<number>(0);
    const [official, setOfficial] = useState<boolean>(false);
    const [selectedUserName, setSelectedUserName] = useState<string>("");
    const [selectedContest, setSelectedContest] = useState<string>("");
    const [userSearch, setUserSearch] = useState<string>("");
    const [contestSearch, setContestSearch] = useState<string>("");
    const [userList, setUserList] = useState<User[]>([]);
    const [contestList, setContestList] = useState<Contest[]>([]);
    const [errors, setErrors] = useState<any>({});

    useEffect(() => {
        if (uid !== undefined) {
            ParticipationService.getParticipation(uid + "/" + cid).then((res) => {
                let participation = res.data;
                setUserId(participation.uid);
                setContestId(participation.cid);
                setScore(participation.score);
                setOfficial(participation.official);
                //setSelectedUserName(submission.user.last_name);
                //setSelectedProblem(submission.problem.name);
            });
        }
        handleAutocompleteUser("");
        handleAutocompleteContest("");
    }, [uid, cid]);

    function submit() {
        if (score < 0) {
            setErrors({score: true});
            return;
        }
        if (uid != null) {
            const participation: Participation = {
                uid: Number(uid),
                cid: Number(cid),
                score,
                official
            };

            ParticipationService.updateParticipation(participation).then((res) => {
                navigate(-1);
            }).catch((res) => {
                setErrors({general: "An error has occurred!"});
            })
        }
        else {
            let val;
            if (userId === -1 && authContext) {
                val = authContext.id;
            }
            else {
                val = userId;
            }

            const participation: Participation = {
                uid: val,
                cid: contestId,
                score,
                official
            };

            ParticipationService.addParticipation(participation).then((res) => {
                navigate(-1);
            }).catch((res) => {
                setErrors({general: "An error has occurred!"});
            })
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

    const handleAutocompleteContest = (name: string) => {
        clearTimeout(timerIDC);

        timerIDC = setTimeout(() => {
            ContestService.getContestAutocomplete(name).then((res) => {
                setContestList(res.data);
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

    const contestNameList = contestList.map((el) => {
        return (
            <p className="panel-block" key={el.id} onClick={() => {setContestId(el.id); setSelectedContest(el.name)}}>
                {el.name}
            </p>
        );
    });

    // @ts-ignore
    // @ts-ignore
    return (
        <div>
            <h1 className="title">{uid != null? "Edit Participation": "Create Participation"}</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    {errors["general"]? (<p className="has-text-danger">{errors["general"]}</p>) : null}
                    { (uid != null || (authContext && authContext.role === "regular"))? null: (
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
                            {(userNameList.length > 0)? userNameList: (<p>No data to show.</p>)}

                            <div className="panel-block">
                                <p>Selected user: {selectedUserName} </p>
                            </div>
                        </nav>) }
                    { (cid != null)? null: (
                        <nav className="panel">
                            <p className="panel-heading">
                                Contest
                            </p>

                            <div className="panel-block">
                                <p className="control has-icons-left">
                                    <input className="input"
                                    type="text"
                                    placeholder="Search"
                                    value={contestSearch}
                                    onChange={(e) => {setContestSearch(e.target.value); handleAutocompleteContest(e.target.value)}}
                                    />
                                    <span className="icon is-left">
                                        <FontAwesomeIcon icon={faSearch} />
                                    </span>
                                </p>
                            </div>
                            {(contestNameList.length > 0)? contestNameList: (<p>No data to show.</p>)}
                            <div className="panel-block">
                                <p>Selected contest: {selectedContest} </p>
                            </div>
                        </nav>
                    )}

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
                        {errors["score"]? (<p className="has-text-danger">Score must be larger than 0!</p>) : null}
                    </div>

                    <label className="checkbox">
                        <input type="checkbox"
                               value={String(official)}
                               onChange={(e) => {setOfficial(e.target.checked)}}/>
                            Official
                    </label>

                    <div className="field is-grouped">
                        <div className="control">
                            <button className="button is-link" onClick={() => submit()}>
                                {uid != null? "Modify Participation": "Add Participation"}
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