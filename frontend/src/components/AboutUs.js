import React from 'react';
import { Container, Col, Row, Card, Button } from 'react-bootstrap';

const AboutUs = () => {
	return (
		<Container className='py-5'>
			<Card>
				<Card.Img
					src='images/team.jpg'
					style={{
						width: '100%',
						height: '40vh',
						objectFit: 'cover',
					}}
				/>
				<Card.Body>
					<Card.Title>About Us</Card.Title>
					<Card.Text>
						Some quick example text to build on the card title and make up the
						bulk of the card's content.
					</Card.Text>
					<Button variant='primary'>Learn More</Button>
					<Button variant='primary'>Meet The Team</Button>
					<Button variant='primary'>Contact Us</Button>
				</Card.Body>
			</Card>
		</Container>
	);
};

export default AboutUs;
