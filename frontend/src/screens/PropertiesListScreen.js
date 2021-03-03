import React, {useEffect, useState} from 'react';
import { Container, Col, Row, Card } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';
import axios from 'axios';
const PropertiesListScreen = () => {

	const [houses, setHouses] = useState([])

	useEffect(() => {
		const fetchHouses = async () => {
			const {data} = await axios.get(`/houses`);
			setHouses(data);
		}
		fetchHouses()
		
	},[])

	return (
		<Container className='py-3'>
			<h1>Our Properties</h1>
			<Row>
				{houses.map((house) => (
					<Col sm={12} md={6} lg={4} xl={3} key={house.id}>
						<Card className='my-2 mx-2' style={{ height: '90%' }}>
							<LinkContainer
								to={`/properties/${house.id}`}
								style={{ cursor: 'pointer' }}
							>
								<Card.Img
									variant='top'
									src={house.image}
									style={{
										width: '100%',
										height: '40vh',
										objectFit: 'cover',
									}}
								/>
							</LinkContainer>
							<Card.Body>
								<LinkContainer
									to={`/properties/${house.id}`}
									style={{ cursor: 'pointer' }}
								>
									<Card.Title>{house.streetName}</Card.Title>
								</LinkContainer>
								<Card.Text>{house.description}</Card.Text>
							</Card.Body>
						</Card>
					</Col>
				))}
			</Row>
		</Container>
	);
};

export default PropertiesListScreen;
