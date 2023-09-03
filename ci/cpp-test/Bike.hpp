#ifndef BIKE_HPP
#define BIKE_HPP

#include "Vehicle.hpp"

class Bike : public Vehicle {
public:
    virtual void getName() override;
};

#endif // BIKE_HPP
