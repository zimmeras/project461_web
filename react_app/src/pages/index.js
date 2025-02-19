import React, { useState } from 'react';
import axios from 'axios';
import PackageList from '../components/packageList';

const Home = () => {

const [result, setResult] = useState(null);
const [version, setVersion] = useState('');
const [pack_name, setPackName] = useState('');
const [packages, setPackages] = useState(null);
const [errormsg, setError] = useState('');
const [errorcode, setCode] = useState('');
const [errorbool, setErrorbool] = useState(false);
const [valid, setValid] = useState(false);

const handleClick = () => {
	axios.get(process.env.REACT_APP_SERVER_URL + '/andrew')
	.then(response => {
		setResult(response.data);
	})
	.catch(error => {
		setError(error.message);
		setCode(error.code);
		setErrorbool(true);
	});
};

const getPackages = () => {
	let token = sessionStorage.getItem("auth_token");
	const request_body = {
		PackageQuery: { Version: version, Name: pack_name },
		headers: { 'X-Authorization': token, 'Content-Type': 'application/json' },
	}
	axios.post(process.env.REACT_APP_SERVER_URL + '/packages', request_body)
	.then(response => {
		setPackages(response.data);
		setValid(true);
	})
	.catch(error => {
		setError(error.message);
        setCode(error.code);
        setErrorbool(true);
	});
};

return (
	<div>
		<div>
			<h1>Welcome to The Package Manager</h1>
				<h4>This website gives users the opportunity to browser a wide range of Node.js packages</h4>
					<p>Use the navigation bar at the top to choose different options for interacting with our package library. Ensure to Log In first. Only authenticated users can interact with our library.</p>
		</div>

		{/* for making sure the flask app is connected */}
		<button onClick={handleClick}>Click me</button>
    	{result && <p>Succesfully connected to Flask with message: {result.message}</p>}

		<br />
		<form onSubmit={getPackages}>
			<label>Version: </label>
            <input
              type="text"
              placeholder="Enter the version you want"
              value={version}
              onChange={(event) => setVersion(event.target.value)}
            />
			<label>Package Name: </label>
            <input
              type="text"
              placeholder="Enter the package name you want"
              value={pack_name}
              onChange={(event) => setPackName(event.target.value)}
              required
            />
			<button type="submit">Get Packages</button>
		</form>
		{valid && <div>
			<PackageList data={packages} />
			</div>}
		{errorbool && <div>
            <p>Error {errorcode}: {errormsg}</p>
            </div>}
	</div>
);
};

export default Home;
