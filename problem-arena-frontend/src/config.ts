import axios from "axios";

export let axiosConfigured = axios.create({baseURL: "/api"});