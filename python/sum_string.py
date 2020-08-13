import libmyrustlib as rs

mylist = list([10, 5, 6, 18, 10])

print("10+12 as a string is %s",rs.sum_as_string(10,12))
print("The length of mylist using python is {}".format(rs.length_list(mylist)))
print("The length of mylist using rust is {}".format(rs.length_list_rust(mylist)))
print("Sorted list {}".format(rs.sort_list(mylist)))
print("List to set {}".format(rs.list_to_set(mylist)))


