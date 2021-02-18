import React from 'react';
import { Container, Col, Row, Card } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';
import housesData from '../data/houses.js';

const PropertiesListScreen = () => {
	return (
		<Container className='py-3'>
			<h1>Our Properties</h1>
			<Row>
				{housesData.map((house) => (
					<Col sm={12} md={6} lg={4} xl={3}>
						<Card className='my-2 mx-2' style={{ height: "90%" }}>
							<LinkContainer to='/propertiesHouse' style={{ cursor: "pointer" }}>
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
								<LinkContainer to='/propertiesHouse' style={{ cursor: "pointer" }}>
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
