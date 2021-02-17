import React from 'react';
import { Container, Col, Row, Card } from 'react-bootstrap';
import housesData from '../data/houses.js';

const PropertiesScreen = () => {
	return (
		<Container className='py-3'>
			<h2>Our Properties</h2>
			<Row>
				{housesData.map((house) => (
					<Col sm={12} md={6} lg={4} xl={3}>
						<Card className='my-2 mx-2'>
							<Card.Img variant='top' src={house.image} />
							<Card.Body>
								<Card.Title>{house.streetName}</Card.Title>
								<Card.Text>{house.description}</Card.Text>
							</Card.Body>
						</Card>
					</Col>
				))}
			</Row>
		</Container>
	);
};

export default PropertiesScreen;
