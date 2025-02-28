import numpy as np 

def separate_matrix(expression):
    expression = expression.replace(' ','')
    if ';' in expression:
        expression = expression.split(';')
        for index,vector in enumerate(expression):
            expression[index] = vector.split(',')
    else:
        expression = expression.split(',')
    arr = np.array(expression)
    for i in range(len(expression)):
        for j in range(len(expression[0])):
            expression[i][j] = float(expression[i][j])
    return arr

operations_map = {
    '+': lambda x,y: x+y,
    '-': lambda x,y: x-y,
    '.': lambda x,y: x.dot(y),
    '*': lambda x,y: x*y,
    '**': lambda x,y: x+y,
    '..': lambda x,y: x+y,
    '/': lambda x,y: x/y,
}

def split_op(expression, operations):
    expression_list = []
    sep = 0
    for index, character in enumerate(expression):
        if character in operations:
            expression_list.append(expression[sep:index-1])
            expression_list.append(expression[index])
            sep = index+1 
    expression_list.append(expression[sep:])
    return expression_list

def separate_expression(expression):
    expression = expression.replace(' ','')
    expr = split_op(expression,('+','-'))
    for index, element in enumerate(expr):
        if any(el in element for el in ['*','/']):
            expr[index] = split_op(element, ('*','/'))
    return expr

def calculate_ls(expression):
    separated_expression = separate_expression(expression)
    for index in range(1,len(separated_expression),2):
        mat1 = separate_matrix(separated_expression[index-1])
        mat2 = separate_matrix(separated_expression[index+1])
        separated_expression[index+1] = operations_map[separated_expression[index]](mat1,mat2)
    return separated_expression[-1]

def calculate(expression_list):
    for lst in expression_list:
        if type(lst) == list:
            calculate_ls(lst)
    return calculate_ls(expression_list)

print(f"Separate matrix:{separate_matrix('1, 5  ,2 ; 2,4,8')}") 
print(f"Separate expression:{separate_expression('1, 5  ,2 ; 2,4*5,8 + 2,2,2;4,1*4,4')}") 
print(f"Separate expression:{calculate('1, 5  ,2 ; 2,4*5,8 + 2,2,2;4,1*4,4')}") 

print("\nWelcome to my matrix calculator!\n")
print("Allowed operators: +, -, * and /\n")
exp = ""
while exp != "e":
    exp = input("> ")
    print(calculate_ls(exp))
