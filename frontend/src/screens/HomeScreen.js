import React from 'react';
import MainHomeWorth from '../components/MainHomeWorth';
import MarketingServices from '../components/MarketingServices';
import AboutUs from '../components/AboutUs';
import { Carousel } from 'react-bootstrap';
import houses from '../data/houses';

const HomeScreen = () => {
	return (
		<>
			<Carousel>
				{houses.map((house) => (
					<Carousel.Item>
						<img
							style={{
								width: '100%',
								minHeight: '75vh',
								maxHeight: '75vh',
								objectFit: 'cover',
							}}
							src={house.image}
							alt={house.streetName}
						/>
						<Carousel.Caption>
							<h2 style={{color: 'white'}}>{house.streetName}</h2>
							<p>{house.description}</p>
						</Carousel.Caption>
					</Carousel.Item>
				))}
			</Carousel>
			<MainHomeWorth />
			<MarketingServices />
			<AboutUs />
		</>
	);
};

export default HomeScreen;
