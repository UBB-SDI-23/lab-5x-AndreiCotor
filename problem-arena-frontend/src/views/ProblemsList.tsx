import {useEffect, useState} from "react";
import {ProblemsService} from "../services/problems-service";
import Problem from "../model/problem";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faStar} from "@fortawesome/free-solid-svg-icons";
import {faStar as faStarBorder} from "@fortawesome/free-regular-svg-icons";
import {useNavigate} from "react-router-dom";

function displayRatingAsStars(rating: number) {
    let stars = [];
    for (let i = 0; i < rating; i++) {
        stars.push(<FontAwesomeIcon icon={faStar} style={{color: "#f9c802"}}/>);
    }
    for (let i = rating; i < 5; i++) {
        stars.push(<FontAwesomeIcon icon={faStarBorder} style={{color: "#f9c802"}}/>);
    }
    return (
        <div>
            {stars}
        </div>
    );
}

export default function ProblemsList() {
    const [problemList, setProblemList] = useState<Problem[]>([]);
    const navigate = useNavigate();

    useEffect(() => {
        ProblemsService.getProblems().then((res) => setProblemList(res.data))
    }, []);

    const tableRows = problemList.map((el, index) => {
        return (<tr key={index} onClick={() => navigate("/problem/" + el.id)}>
            <td>{index + 1}</td>
            <td>{el.name}</td>
            <td>{el.author}</td>
            <td>{el.contest}</td>
            <td>{displayRatingAsStars(el.rating)}</td>
        </tr>);
    });

    return (
        <div>
            <h1 className="title">Problem List</h1>
            <table className="table is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th><abbr title="Index">#</abbr></th>
                        <th>Name</th>
                        <th>Author</th>
                        <th>Contest</th>
                        <th>Rating</th>
                    </tr>
                </thead>
                <tbody>
                    {tableRows}
                </tbody>
            </table>
        </div>
    );
}