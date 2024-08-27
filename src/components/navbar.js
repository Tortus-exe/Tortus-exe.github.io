import { Link } from "react-router-dom";
import categories from "../Content/categories.json";
import "./navbar.css";
import React, { useState } from "react";

function MakeButton(data, num) {
    const [active, setActive] = useState(false);
    return (
      <Link to={`/${data.uri}`}>
        <button key={num} className={"navbarButton" + (active ? " activeNav" : "")}
          onMouseOver={() => {setActive(true)}}
          onMouseLeave={() => {setActive(false)}}>
              <div className={"navbarButtonText" + (active ? " activeNav" : "")}>
                  {data.title}
              </div>
        </button>
      </Link>
    );
}

function BackButton() {
    const [active, setActive] = useState(false);
    return (
      <Link to={'..'}>
        <button key={4} className={"navbarButton" + (active ? " activeNav" : "")}
          onMouseOver={() => {setActive(true)}}
          onMouseLeave={() => {setActive(false)}}>
              <div className={"navbarButtonText" + (active ? " activeNav" : "")}>
                  Back
              </div>
        </button>
      </Link>
    );
}

function Navbar() {
  let options = categories.categories;

  return (
    <div>
      <div className="backbutton">
        {BackButton()}
      </div>
      <div className="navbar">
        {options.map((a, b) => MakeButton(a, b))}
      </div>
    </div>
  );
}

export default Navbar;