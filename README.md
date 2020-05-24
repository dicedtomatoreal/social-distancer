# social_distancer

This CLI App allows you to take a set of people, generate how close they are to a certain person and how close they are by certain meters.

# Usage
```
social_distancer --people <people> --distance <distance> --regulation <regulation>
```
Or
```
social_distancer -p <people> -d <distance> -r <regulation>
```

# Options
+------------+-----------------------------------------------------------+
| Options    | Description                                               |
+------------+-----------------------------------------------------------+
| people     | Number of people to check                                 |
+------------+-----------------------------------------------------------+
| distance   | The max distance to choose from, for example 6 (6 meters) |
+------------+-----------------------------------------------------------+
| regulation | The distancing regulation, for example 3 (3 meters)       |
+------------+-----------------------------------------------------------+

# Example
```
social_distancer -p 10 -d 6 -r 3
# outputs people who are close and not close and some statistic 
```