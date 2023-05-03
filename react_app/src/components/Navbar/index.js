import React from "react";
import { Nav, NavLink, NavMenu }
	from "./NavbarElements";
import logo from "../../images/purdue.png";

/*
THIS COMPONENT WAS ADAPTED FROM THE CODE FROM GEEKSFORGEEKS.COM
https://www.geeksforgeeks.org/how-to-create-a-multi-page-website-using-react-js/#

*/

const Navbar = () => {
return (
	<>
	<Nav>
		<NavMenu>
		<NavLink to="/">
			<img src={logo} alt="Home"></img>
		</NavLink>
		<NavLink to="/packages">
			Packages
		</NavLink>
		<NavLink to="/upload">
			Upload
		</NavLink>
		<NavLink to="/rating">
			Rating
		</NavLink>
		<NavLink to="/login">
			Log In Here
		</NavLink>
		<NavLink to="/delete">
			Delete
		</NavLink>
		</NavMenu>
	</Nav>
	</>
);
};

export default Navbar;
