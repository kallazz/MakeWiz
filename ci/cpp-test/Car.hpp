#ifndef CAR_HPP
#define CAR_HPP

#include "Vehicle.hpp"

class Car : public Vehicle {
public:
    virtual void getName() override;
};

#endif // CAR_HPP
