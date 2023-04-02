import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faStar} from "@fortawesome/free-solid-svg-icons";
import {faStar as faStarBorder} from "@fortawesome/free-regular-svg-icons";

export default function RatingDisplay(props: any) {
    let stars = [];
    for (let i = 0; i < props.rating; i++) {
        stars.push(<FontAwesomeIcon icon={faStar} style={{color: "#f9c802"}}/>);
    }
    for (let i = props.rating; i < 5; i++) {
        stars.push(<FontAwesomeIcon icon={faStarBorder} style={{color: "#f9c802"}}/>);
    }

    return (
      <div>
          {stars}
      </div>
    );
}