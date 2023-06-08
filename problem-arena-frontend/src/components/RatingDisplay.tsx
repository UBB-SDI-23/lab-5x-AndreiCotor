import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faStar} from "@fortawesome/free-solid-svg-icons";
import {faStar as faStarBorder} from "@fortawesome/free-regular-svg-icons";
import {useMediaQuery} from "react-responsive";

export default function RatingDisplay(props: any) {
    const isLarger769 = useMediaQuery({ query: `(min-width: 769px)` });
    const isSmaller911 = useMediaQuery({ query: `(max-width: 911px)` });
    const isLarger1024 = useMediaQuery({ query: `(min-width: 1024px)` });
    const isSmaller1096 = useMediaQuery({ query: `(max-width: 1096px)` });

    let stars = [];
    for (let i = 0; i < props.rating; i++) {
        stars.push(<FontAwesomeIcon icon={faStar} style={{color: "#f9c802"}}/>);
    }
    for (let i = props.rating; i < 5; i++) {
        stars.push(<FontAwesomeIcon icon={faStarBorder} style={{color: "#f9c802"}}/>);
    }

    return (
      <div>
          {((isLarger769 && isSmaller911) || (isLarger1024 && isSmaller1096))? (props.rating) : stars}
      </div>
    );
}