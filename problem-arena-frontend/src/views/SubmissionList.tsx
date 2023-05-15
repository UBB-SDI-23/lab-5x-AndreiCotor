import {useContext, useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";
import {SubmissionWithNameDTO} from "../model/submission";
import {SubmissionService} from "../services/submission-service";
import {AuthContext} from "../contexts/AuthContext";

export default function SubmissionList() {
    const [submissionList, setSubmissionList] = useState<SubmissionWithNameDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();
    const [error, setError] = useState<string>("");
    const { authContext } = useContext(AuthContext);

    useEffect(() => {
        SubmissionService.getSubmissions(pagination).then((res) => {
            if (res.data.length > 0) {
                setSubmissionList(res.data.map((el) => {
                    return {
                        id: el.id,
                        source_code: el.source_code,
                        score: el.score,
                        language: el.language,
                        user: el.user.last_name,
                        problem: el.problem.name,
                        uid: el.user.id
                    }
                }));
            }
        }).catch((res) => setError("An error has occurred!"))
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteSubmission = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            try {
                await SubmissionService.deleteSubmission(id);
            }
            catch (err) {
                setError("An error has occurred!");
            }
            forceUpdate();
        }
    }

    const previousPage = () => {
        if (submissionList.length > 0) {
            setPagination({first_id: submissionList[0].id, last_id: submissionList[submissionList.length - 1].id, limit: 10, direction: -1});
        }
    }

    const nextPage = () => {
        if (submissionList.length > 0) {
            setPagination({first_id: submissionList[0].id, last_id: submissionList[submissionList.length - 1].id, limit: 10, direction: 1});
        }
    }

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">Submission List</h1>
                </div>
                <div className="column">
                    {authContext? (<button className="button is-pulled-right is-link" onClick={() => navigate("/submission/create")}>
                        Add Submission
                    </button>): null}
                </div>
            </div>
            <br/>
            <p className="has-text-danger">{error}</p>
            <Table columns={["Score", "Language", "Problem"]}
                   properties={["score", "language", "problem"]}
                   elements={submissionList}
                   path={"/submission"}
                   deleteFunction={(id) => deleteSubmission(id)}
                   creator="user"
                   uid="uid"
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}