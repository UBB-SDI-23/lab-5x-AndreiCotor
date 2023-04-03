import axios from "axios";

export let axiosConfigured = axios.create({baseURL: "/api"});
//export let axiosConfigured = axios.create({baseURL: "http://localhost:8080"});