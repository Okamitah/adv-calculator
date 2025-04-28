def separate_expression(expression: str):

    """Function that takes a string and gives back a list"""

    expression = expression.replace(" ","")

    ops = ['+', '-','*', '/']

    ls = split_ops(expression, ops)

    return ls


def split_ops(expression, operations):

    expression_list = []
    sep = 0

    for index, character in enumerate(expression):
        if character in operations:
            expression_list.append(expression[sep:index])
            expression_list.append(character)
            sep = index+1 

    expression_list.append(expression[sep:])

    return expression_list

# Yooooo
# Test Test

operations_map = {
    '+': lambda x,y : float(x) + float(y),
    '-': lambda x,y : float(x) - float(y),
    '*': lambda x,y : float(x) * float(y),
    '/': lambda x,y : float(y) / float(x),
}

def find_closing_parentheses(start:int, separated_expression:list):
    open_parens = 1 
    index = 1
    for ind, elem in enumerate(separated_expression[start+1:]):
        if elem == '(':
            open_parens += 1 
        elif elem == ')':
            open_parens -= 1 
        if open_parens == 0:
            index += ind
            break
    return index+start


# idk what is was
## *Eeee*


def calculate_list(expression: list):

    muldiv = ('*','/')

    for i in range(1,len(expression),2):
        if expression[i] in muldiv:
            expression[i+1] = operations_map[expression[i]](expression[i+1],expression[i-1])
            expression[i-1] = '%'
            expression[i] = '%'
        
    expression = [ch for ch in expression if ch != '%']

    for i in range(1,len(expression),2):
        expression[i+1] = operations_map[expression[i]](expression[i-1],expression[i+1])

    return expression[-1]


def calculate(expression: str):
    separated_expression = separate_expression(expression)
    return calculate_list(separated_expression)



print("\nWelcome to my basic calculator!\n")
print("Allowed operators: +, -, * and /\n")
exp = ""
while exp != "exit":
    exp = input("> ")
    print(calculate(exp))
