import React, { useState, useEffect } from 'react';
import { Container, Image } from 'react-bootstrap';
import axios from 'axios';

const PropertiesScreen = ({ match }) => {
	const houseId = match.params.id;

	const [house, setHouse] = useState([]);

	useEffect(() => {
		const fetchAHouse = async () => {
			const {data} = await axios.get(`/houses/${houseId}`);
			setHouse(data);
		};
		fetchAHouse();
	}, []);
	return (
		<Container className='py-3'>
			<h1>{house.street_name}</h1>
			<Image
				src={house.image}
				style={{
					width: '100%',
					height: '100%',
					objectFit: 'cover',
				}}
			/>
			<h2 className='py-3'>$ {house.price}</h2>
			<h5>{house.long_description}</h5>
		</Container>
	);
};

export default PropertiesScreen;
