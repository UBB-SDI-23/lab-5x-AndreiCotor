import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {PaginationDTO} from "../model/PaginationDTO";
import {NewSubmission, Submission, SubmissionDTO} from "../model/submission";

export const SubmissionService = {
    getSubmissions: (pagination: PaginationDTO): Promise<AxiosResponse<SubmissionDTO[]>> => {
        return axiosConfigured.get("/submission", {params: pagination});
    },

    getSubmission: (id: string): Promise<AxiosResponse<SubmissionDTO>> => {
        return axiosConfigured.get("/submission/" + id);
    },

    updateSubmission: (submission: Submission): Promise<AxiosResponse> => {
        return axiosConfigured.put("/submission", submission, {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    },

    addSubmission: (submission: NewSubmission): Promise<AxiosResponse> => {
        return axiosConfigured.post("/submission", submission, {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    },

    deleteSubmission: (id: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/submission/" + id, {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    }
}