import React from 'react';
import { Container, Form, Button } from 'react-bootstrap';

const ContactUsScreen = () => {
	return (
		<Container className='py-3'>
			<h1>Contact Us</h1>
			<Form>
				<Form.Group>
					<Form.Label>Full Name</Form.Label>
					<Form.Control type='text' placeholder='Enter your name' size='lg'/>
					<Form.Text className='text-muted'>
						Please enter your preferred full name to address you.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Email Address</Form.Label>
					<Form.Control type='email' placeholder='Enter email' size='lg'/>
					<Form.Text className='text-muted'>
						We'll never share your email with anyone else.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Phone Number</Form.Label>
					<Form.Control
						type='tel'
						placeholder='555-555-5555'
						pattern='[0-9]{3}-[0-9]{3}-[0-9]{4}'
                        size='lg'
					/>
					<Form.Text className='text-muted'>
						Please enter your preferred full name to address you.
					</Form.Text>
				</Form.Group>
				<Form.Group>
					<Form.Label>Message</Form.Label>
					<Form.Control type='text' placeholder='' size='lg'/>
					<Form.Text className='text-muted'>
						Please send us a short message on any houses you liked or any
						request you may have.
					</Form.Text>
				</Form.Group>
			</Form>
			<Button variant='primary' type='submit' block size='lg'>
				<h1 style={{color: "white"}}>Submit</h1>
			</Button>
		</Container>
	);
};

export default ContactUsScreen;
