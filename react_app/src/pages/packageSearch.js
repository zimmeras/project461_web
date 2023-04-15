import React, { useState, useEffect } from 'react';
import axios from 'axios';

const PackageSearch = () => {
  const [id, setId] = useState('');
  const [search, setSearch] = useState(false);
  const [packagedata, setPackagedata] = useState([]);
  const token = sessionStorage.getItem('auth_token');
  const handleSubmit = (event) => {
    event.preventDefault();
    useEffect(() => {
        axios.put(`https://localhost:8080/package/${id}`, {
          headers: {
            'Content-Type': 'application/json',
            'X-Authorization': token
          }
        })
          .then(response => {
            setPackagedata(response.data);
            setSearch(true);
          })
          .catch(error => {
            console.log(error);
          });
        }, []);
    };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Search packages"
          value={id}
          onChange={(e) => setId(e.target.value)}
        />
        <button type="submit">Search</button>
      </form>
      <div>
        {search && <ul>
        {packagedata.map(post => (
          <li key={post.id}>{post.title}</li>
        ))}
      </ul>}
      </div>
    </div>
  );
}

export default PackageSearch;