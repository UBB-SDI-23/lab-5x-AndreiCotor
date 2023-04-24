import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.sass';
import reportWebVitals from './reportWebVitals';
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import NavMenu from "./components/NavMenu";
import NavBar from "./components/NavBar";
import ProblemsList from "./views/ProblemsList";
import ProblemView from "./views/ProblemView";
import ProblemDetailsForm from "./views/ProblemDetailsForm";
import ProblemsListBySuccessRate from "./views/ProblemListBySuccessRate";
import ContestList from "./views/ContestList";
import UserList from "./views/UserList";
import SubmissionList from "./views/SubmissionList";
import ParticipationList from "./views/ParticipationList";
import ContestDetailsForm from "./views/ContestDetailsForm";
import UserDetailsForm from "./views/UserDetailsForm";
import SubmissionDetailsForm from "./views/SubmissionDetailsForm";
import ParticipatesDetailsForm from "./views/ParticipatesDetailsForm";
import ContestView from "./views/ContestView";
import UserView from "./views/UserView";
import SubmissionView from "./views/SubmissionView";
import ParticipationView from "./views/ParticipationView";

const router = createBrowserRouter([{
    path: '/',
    element: <div>Hello world!</div>
}, {
    path: '/problems',
    element: <ProblemsList />
}, {
    path: '/problem/create',
    element: <ProblemDetailsForm />
}, {
    path: '/problem/edit/:id',
    element: <ProblemDetailsForm />
}, {
    path: '/problem/:id',
    element: <ProblemView />
}, {
    path: '/problems-by-success-rate',
    element: <ProblemsListBySuccessRate/>
}, {
    path: '/contests',
    element: <ContestList />
}, {
    path: '/users',
    element: <UserList />
}, {
    path: '/submissions',
    element: <SubmissionList/>
}, {
    path: '/participations',
    element: <ParticipationList/>
}, {
    path: '/contest/create',
    element: <ContestDetailsForm/>
}, {
    path: '/contest/edit/:id',
    element: <ContestDetailsForm/>
}, {
    path: '/user/create',
    element: <UserDetailsForm/>
}, {
    path: '/user/edit/:id',
    element: <UserDetailsForm/>
}, {
    path: '/submission/create',
    element: <SubmissionDetailsForm/>
}, {
    path: '/submission/edit/:id',
    element: <SubmissionDetailsForm/>
}, {
    path: '/participation/create',
    element: <ParticipatesDetailsForm/>
}, {
    path: '/participation/edit/:uid/:cid',
    element: <ParticipatesDetailsForm/>
}, {
    path: '/contest/:id',
    element: <ContestView/>
}, {
    path: '/user/:id',
    element: <UserView/>
}, {
    path: '/submission/:id',
    element: <SubmissionView/>
}, {
    path: '/participation/:uid/:cid',
    element: <ParticipationView/>
}]);

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);

root.render(
    <React.StrictMode>
        <NavBar/>
        <div className="columns" style={{height: "100%"}}>
            <div className="column is-2" style={{height: "100%"}}><NavMenu/></div>
            <div className="column box mt-2"><RouterProvider router={router}/></div>
        </div>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
