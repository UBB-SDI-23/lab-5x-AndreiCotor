import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";
import {Submission} from "../model/submission";
import {SubmissionService} from "../services/submission-service";

export default function SubmissionList() {
    const [submissionList, setSubmissionList] = useState<Submission[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();

    useEffect(() => {
        SubmissionService.getSubmissions(pagination).then((res) => {
            if (res.data.length > 0) {
                setSubmissionList(res.data);
            }
        })
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteSubmission = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await SubmissionService.deleteSubmission(id);
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
                    <button className="button is-pulled-right is-link" onClick={() => navigate("/submission/create")}>
                        Add Submission
                    </button>
                </div>
            </div>
            <Table columns={["Score", "Language"]}
                   properties={["score", "language"]}
                   elements={submissionList}
                   path={"/submission"}
                   deleteFunction={(id) => deleteSubmission(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}