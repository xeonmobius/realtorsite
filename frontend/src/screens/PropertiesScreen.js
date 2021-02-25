import React, { useState, useEffect } from 'react';
import { Container, Image } from 'react-bootstrap';
import axios from 'axios';
import { useParams } from 'react-router';

const PropertiesScreen = ({ match }) => {
	const houseId = match.params.id;

	const [house, setHouse] = useState([]);

	useEffect(() => {
		const fetchAHouse = async () => {
			const housez = await axios.get(`/houses/${houseId}`);
			setHouse(housez.data);
		};
		fetchAHouse();
	}, []);
	return (
		<Container className='py-3'>
			<h1>{house.streetName}</h1>
			<Image
				src={house.image}
				style={{
					width: '100%',
					height: '100%',
					objectFit: 'cover',
				}}
			/>
			<h2 className='py-3'>$ {house.price}</h2>
			<h5>{house.longDescription}</h5>
		</Container>
	);
};

export default PropertiesScreen;
