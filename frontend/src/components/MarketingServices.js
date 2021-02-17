import React from 'react';
import { Container, Col, Row, Card, Button } from 'react-bootstrap';

const MarketingServices = () => {
	return (
		<Container className='py-5'>
			<Card>
				<Card.Img
					src='images/marketing.jpg'
					style={{
						width: '100%',
						height: '40vh',
						objectFit: 'cover',
					}}
				/>
				<Card.Body>
					<Card.Title>Marketing Services</Card.Title>
					<Card.Text>
						Some quick example text to build on the card title and make up the
						bulk of the card's content.
					</Card.Text>
					<Button variant='primary'>Go somewhere</Button>
				</Card.Body>
			</Card>
		</Container>
	);
};

export default MarketingServices;
