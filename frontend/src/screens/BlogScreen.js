import React, { useEffect, useState } from 'react';
import { Container, Row } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';
import axios from 'axios';

const BlogScreen = () => {
	const [blogPosts, setBlogPosts] = useState([]);
	useEffect(() => {
		const fetchBlog = async () => {
			try {
				const { data } = await axios.get('/blog');
				setBlogPosts(data);
			} catch (e) {
				console.log(e);
			}
		};
		fetchBlog();
	}, []);

	return (
		<Container className='py-3'>
			<h1>Blog</h1>
			{blogPosts.map((blog) => (
				<>
					<h2>{blog.title}</h2>
					<h5>Written by {blog.author}</h5>
					<p>Posted on {blog.date}</p>
					<h3>{blog.text}</h3>
				</>
			))}
		</Container>
	);
};

export default BlogScreen;
