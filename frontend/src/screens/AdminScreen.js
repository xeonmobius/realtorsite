import React, { useState, useEffect } from 'react';
import { Container } from 'react-bootstrap';
import axios from 'axios';


const AdminScreen = () => {
	const [auth, setAuth] = useState(false);

	useEffect(() => {
        const requestAccess = async () => {
            const {data} = await axios.get('/auth', {withCredentials: true});
            console.log(data);
            setAuth(data);
        }

        requestAccess();
    }, []);

	return (
		<Container className='py-3'>
			{!auth ? (
				<h1>Not authorized. Restricted Access</h1>
			) : (
				<>
					<h1>Edit Properties</h1>
					<h1>Edit Team</h1>
				</>
			)}
		</Container>
	);
};

export default AdminScreen;
