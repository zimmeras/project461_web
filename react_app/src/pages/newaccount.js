import React, { useState } from 'react';
import axios from 'axios';
import { Link } from 'react-router-dom';

const NewAccount = () => {
  const [username, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confpassword, setConfirm] = useState('');
  const [newaccconf, setNewAccConf] = useState(false);
  const [matchpassword, setPassMatch] = useState(false);
  const [errormsg, setError] = useState('');
  const [errorcode, setCode] = useState('');
  const [errorbool, setErrorbool] = useState(false);

  const handleSubmit = (event) => {
    event.preventDefault();
    // Here you can add code to submit the form to your backend
    if(password === confpassword) {
      setPassMatch(false);
        console.log('Email:', username);
        console.log('Password:', password);
        const data = {
          "User": {
            "name": {username},
            "isAdmin": true
          },
          "Secret": {
            "password": {password}
          }
        }
        
    
        axios.post(process.env.REACT_APP_SERVER_URL + '/newuser', { data }, {
          headers: {
            'Content-Type': 'application/json'
          }
        })
          .then(response => {
            setErrorbool(false);
            setNewAccConf(true);
            console.log(response.data);
          })
          .catch(error => {
            setError(error.message);
            setCode(error.code);
            setErrorbool(true);
          });
    }
    else {
        setPassMatch(true);
        console.log("passwords do not match");
    }
  };

  return (
    <div>
      {!newaccconf && <div>
        <h1>Create a New Account:</h1>
        <form onSubmit={handleSubmit}>
          <div>
            <label>Username: </label>
            <input
              type="text"
              placeholder="Enter your username"
              value={username}
              onChange={(event) => setEmail(event.target.value)}
              required
            />
          </div>
          <br></br>
          <div>
            <label>Password:  </label>
            <input
              type="password"
              placeholder="Enter your password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              required
            />
          </div>
          <br></br>
          <div>
            <label>Confirm Password: </label>
            <input 
              type="password"
              placeholder="Confirm your password"
              value={confpassword}
              onChange={(event) => setConfirm(event.target.value)}
              required
              />
          </div>
          <button type="submit">Sign In</button>
        </form>
      </div>}
      {newaccconf && <div><p>New Account Created!</p>
      <Link to="/login">
			Click here to Login </Link>
      </div>}
      {errorbool && <div>
            <p>Error {errorcode}: {errormsg}</p>
            </div>}
      {matchpassword && <div><p>those passwords do not match! Try Again</p></div>}
    </div>
  );
}

export default NewAccount;
